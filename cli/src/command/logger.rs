use secretpass_core::{LogItem, LogSource};
use std::collections::VecDeque;

const MAX_BUFFER_SIZE: usize = 1024;

#[derive(Default)]
pub struct Logger {
    count: usize,
    queue: VecDeque<LogItem>,
}

impl Logger {
    pub fn push(&mut self, source: LogSource, line: String) {
        self.queue.push_back(LogItem(
            self.count,
            chrono::Utc::now().to_rfc3339(),
            source,
            line,
        ));

        if self.queue.len() > MAX_BUFFER_SIZE {
            self.queue.pop_front();
        }

        self.count += 1;
    }

    pub fn get_logs(&self, cursor: usize) -> Vec<LogItem> {
        let size = self.count - cursor;
        let skippable = if self.queue.len() > size {
            self.queue.len() - size
        } else {
            0
        };

        self.queue.iter().skip(skippable).cloned().collect()
    }
}
