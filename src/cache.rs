use super::FontRef;

pub struct FontCache<T> {
    entries: Vec<Entry<T>>,
    max_entries: usize,
    epoch: u64,
}

impl<T> FontCache<T> {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            epoch: 0,
            max_entries,
        }
    }

    pub fn get<'a>(&'a mut self, font: &FontRef, mut f: impl FnMut(&FontRef) -> T) -> (u64, &'a T) {
        let (found, index) = self.find(font);
        if found {
            let entry = &mut self.entries[index];
            entry.epoch = self.epoch;
            (entry.id, &entry.data)
        } else {
            self.epoch += 1;
            let data = f(font);
            let id = font.key.value();
            if index == self.entries.len() {
                self.entries.push(Entry {
                    epoch: self.epoch,
                    id,
                    data,
                });
                let entry = self.entries.last().unwrap();
                (id, &entry.data)
            } else {
                let entry = &mut self.entries[index];
                entry.epoch = self.epoch;
                entry.id = id;
                entry.data = data;
                (id, &entry.data)
            }
        }
    }

    fn find(&self, font: &FontRef) -> (bool, usize) {
        let mut lowest = 0;
        let mut lowest_epoch = self.epoch;
        let id = font.key.value();
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.id == id {
                return (true, i);
            }
            if entry.epoch < lowest_epoch {
                lowest_epoch = entry.epoch;
                lowest = i;
            }
        }
        if self.entries.len() < self.max_entries {
            (false, self.entries.len())
        } else {
            (false, lowest)
        }
    }
}

struct Entry<T> {
    epoch: u64,
    id: u64,
    data: T,
}
