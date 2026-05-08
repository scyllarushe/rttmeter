use std::time::Duration;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProbeStatistics {
    sent: u32,
    received: u32,
    total_rtt_ms: f64,
    last_rtt_ms: Option<f64>,
    best_rtt_ms: Option<f64>,
    worst_rtt_ms: Option<f64>,
}

impl ProbeStatistics {
    pub fn record_probe_sent(&mut self) {
        self.sent += 1;
    }

    pub fn record_reply(&mut self, rtt: Duration) {
        let rtt_ms = duration_to_millis(rtt);

        self.received += 1;
        self.total_rtt_ms += rtt_ms;
        self.last_rtt_ms = Some(rtt_ms);
        self.best_rtt_ms = Some(match self.best_rtt_ms {
            Some(best) => best.min(rtt_ms),
            None => rtt_ms,
        });
        self.worst_rtt_ms = Some(match self.worst_rtt_ms {
            Some(worst) => worst.max(rtt_ms),
            None => rtt_ms,
        });
    }

    pub fn sent(&self) -> u32 {
        self.sent
    }

    pub fn received(&self) -> u32 {
        self.received
    }

    pub fn loss_percentage(&self) -> f64 {
        if self.sent == 0 {
            return 0.0;
        }

        ((self.sent - self.received) as f64 / self.sent as f64) * 100.0
    }

    pub fn last_rtt_ms(&self) -> Option<f64> {
        self.last_rtt_ms
    }

    pub fn average_rtt_ms(&self) -> Option<f64> {
        if self.received == 0 {
            None
        } else {
            Some(self.total_rtt_ms / self.received as f64)
        }
    }

    pub fn best_rtt_ms(&self) -> Option<f64> {
        self.best_rtt_ms
    }

    pub fn worst_rtt_ms(&self) -> Option<f64> {
        self.worst_rtt_ms
    }
}

fn duration_to_millis(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

#[cfg(test)]
mod tests {
    use super::ProbeStatistics;
    use std::time::Duration;

    #[test]
    fn statistics_track_loss_and_round_trip_values() {
        let mut stats = ProbeStatistics::default();

        stats.record_probe_sent();
        stats.record_reply(Duration::from_micros(2_100));
        stats.record_probe_sent();
        stats.record_reply(Duration::from_micros(4_900));
        stats.record_probe_sent();

        assert_eq!(stats.sent(), 3);
        assert_eq!(stats.received(), 2);
        assert!((stats.loss_percentage() - 33.333_333).abs() < 0.001);
        assert!((stats.last_rtt_ms().expect("last RTT") - 4.9).abs() < 0.001);
        assert!((stats.average_rtt_ms().expect("average RTT") - 3.5).abs() < 0.001);
        assert!((stats.best_rtt_ms().expect("best RTT") - 2.1).abs() < 0.001);
        assert!((stats.worst_rtt_ms().expect("worst RTT") - 4.9).abs() < 0.001);
    }

    #[test]
    fn statistics_without_replies_leave_rtt_values_empty() {
        let mut stats = ProbeStatistics::default();

        stats.record_probe_sent();
        stats.record_probe_sent();

        assert_eq!(stats.sent(), 2);
        assert_eq!(stats.received(), 0);
        assert_eq!(stats.loss_percentage(), 100.0);
        assert_eq!(stats.last_rtt_ms(), None);
        assert_eq!(stats.average_rtt_ms(), None);
        assert_eq!(stats.best_rtt_ms(), None);
        assert_eq!(stats.worst_rtt_ms(), None);
    }
}
