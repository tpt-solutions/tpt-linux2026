#[cfg(test)]
mod tests {
    use tpt_l_journal_oracle::ingest::{parse_entry, parse_stream};
    use tpt_l_journal_oracle::oracle::{
        DiagnosticPipeline, JournalEntry, LlmBackend, MockBackend, OracleError, build_prompt,
    };

    const SAMPLE: &str = "PRIORITY=3\n\
        _SYSTEMD_UNIT=sshd.service\n\
        MESSAGE=Failed password for root from 1.2.3.4 port 22\n";

    #[test]
    fn parse_single_entry() {
        let e = parse_entry(SAMPLE).unwrap();
        assert_eq!(e.unit(), Some("sshd.service"));
        assert_eq!(e.priority(), Some(3));
        assert!(e.message().unwrap().contains("Failed password"));
    }

    #[test]
    fn parse_multi_entry_stream() {
        let input = format!("{}\n\n{}", SAMPLE, "MESSAGE=second entry\nPRIORITY=6\n");
        let entries = parse_stream(&input);
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn prompt_contains_entries() {
        let e = parse_entry(SAMPLE).unwrap();
        let prompt = build_prompt(std::slice::from_ref(&e));
        assert!(prompt.contains("sshd.service"));
        assert!(prompt.contains("Failed password"));
    }

    struct FailingBackend;
    impl LlmBackend for FailingBackend {
        fn generate(&self, _: &str) -> Result<String, OracleError> {
            Err(OracleError::Backend("boom".into()))
        }
    }

    #[test]
    fn pipeline_uses_mock() {
        let entries =
            vec![parse_entry(SAMPLE).unwrap(), JournalEntry { fields: Default::default() }];
        let pipe = DiagnosticPipeline::new(MockBackend::new("all good"), 1);
        let out = pipe.run(&entries).unwrap();
        assert_eq!(out, vec!["all good".to_string(), "all good".to_string()]);
    }

    #[test]
    fn pipeline_propagates_error() {
        let e = parse_entry(SAMPLE).unwrap();
        let pipe = DiagnosticPipeline::new(FailingBackend, 1);
        assert!(pipe.run(std::slice::from_ref(&e)).is_err());
    }

    #[test]
    fn stream_iterator() {
        let e = parse_entry(SAMPLE).unwrap();
        let pipe = DiagnosticPipeline::new(MockBackend::new("diag"), 1);
        let collected: Vec<_> = pipe.stream(std::slice::from_ref(&e)).collect();
        assert_eq!(collected.len(), 1);
        assert_eq!(collected[0].as_deref(), Ok("diag"));
    }
}
