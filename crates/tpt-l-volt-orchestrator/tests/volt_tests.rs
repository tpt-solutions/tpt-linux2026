#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tpt_l_volt_orchestrator::*;

    fn fake_fs() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("cpu0/cpufreq/scaling_governor".into(), "powersave".into());
        m.insert("cpu0/cpufreq/scaling_max_freq".into(), "2400000".into());
        m.insert("cpu1/cpufreq/scaling_governor".into(), "powersave".into());
        m.insert("cpu1/cpufreq/scaling_max_freq".into(), "2400000".into());
        m
    }

    #[test]
    fn reads_sysfs_values() {
        let iface = CpufreqInterface::with_store(fake_fs());
        assert_eq!(iface.governor(0).unwrap(), "powersave");
        assert_eq!(iface.max_freq(0).unwrap(), 2_400_000);
    }

    #[test]
    fn writes_sysfs_values() {
        let iface = CpufreqInterface::with_store(fake_fs());
        iface.set_governor(0, "performance").unwrap();
        assert_eq!(iface.governor(0).unwrap(), "performance");
        iface.set_max_freq(1, 1_000_000).unwrap();
        assert_eq!(iface.max_freq(1).unwrap(), 1_000_000);
    }

    #[test]
    fn policy_from_load() {
        assert_eq!(Policy::from_load(0.9), Policy::Performance);
        assert_eq!(Policy::from_load(0.3), Policy::Powersave);
        assert_eq!(Policy::from_load(0.7), Policy::Performance);
    }

    #[test]
    fn apply_policy_drives_governors() {
        let iface = CpufreqInterface::with_store(fake_fs());
        apply_policy(&iface, 0..2, Policy::Performance).unwrap();
        assert_eq!(iface.governor(0).unwrap(), "performance");
        assert_eq!(iface.governor(1).unwrap(), "performance");
    }
}
