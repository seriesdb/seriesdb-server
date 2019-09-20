use seriesdb::table::Table as InnerTable;

type Key = Vec<u8>;
type Value = Vec<u8>;

pub struct Table<'a> {
    inner: InnerTable<'a>,
}

impl<'a> Table<'a> {
    pub fn new(inner: InnerTable) -> Table {
        Table { inner }
    }

    #[inline]
    pub fn set_rows(&self, keys: &Vec<Key>, values: &Vec<Value>) {
        let mut batch = self.inner.batch();
        let size = keys.len();
        for i in 0..size {
            batch.put(&keys[i], &values[i]).unwrap();
        }
        self.inner.write(batch).unwrap();
    }

    #[inline]
    pub fn delete_rows_since(&self, key: &Key, limit: u32) {
        let mut to_key: Vec<u8> = vec![];
        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek(key);
        while iter.is_valid() {
            if count > limit {
                break;
            }
            to_key = iter.key().unwrap().to_vec();
            iter.next();
            count += 1;
        }
        if to_key.len() > 0 {
            let mut batch = self.inner.batch();
            batch.delete_range(key, to_key).unwrap();
            self.inner.write(batch).unwrap();
        }
    }

    #[inline]
    pub fn get_first_row(&self) -> Option<(Key, Value)> {
        let mut iter = self.inner.iter();
        iter.seek_to_first();
        if iter.is_valid() {
            Some((iter.key().unwrap().to_vec(), iter.value().unwrap().to_vec()))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_last_row(&self) -> Option<(Key, Value)> {
        let mut iter = self.inner.iter();
        iter.seek_to_last();
        if iter.is_valid() {
            Some((iter.key().unwrap().to_vec(), iter.value().unwrap().to_vec()))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_boundary_rows(&self) -> Option<(Key, Value, Key, Value)> {
        let mut iter = self.inner.iter();
        iter.seek_to_first();
        if iter.is_valid() {
            let first_key = iter.key().unwrap().to_vec();
            let first_value = iter.value().unwrap().to_vec();
            iter.seek_to_last();
            let last_key = iter.key().unwrap().to_vec();
            let last_value = iter.value().unwrap().to_vec();
            Some((first_key, first_value, last_key, last_value))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_rows_since(&self, key: &Key, limit: u32) -> (Vec<Key>, Vec<Value>) {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek(key);
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            keys.push(iter.key().unwrap().to_vec());
            values.push(iter.value().unwrap().to_vec());
            iter.next();
            count += 1;
        }

        (keys, values)
    }

    #[inline]
    pub fn get_rows_until(&self, key: &Key, limit: u32) -> (Vec<Key>, Vec<Value>) {
        let mut reversed_keys = Vec::new();
        let mut reversed_values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek_for_prev(key);
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            reversed_keys.push(iter.key().unwrap().to_vec());
            reversed_values.push(iter.value().unwrap().to_vec());
            iter.prev();
            count += 1;
        }
        reversed_keys.reverse();
        reversed_values.reverse();

        (reversed_keys, reversed_values)
    }

    #[inline]
    pub fn get_rows_until_last(&self, limit: u32) -> (Vec<Key>, Vec<Value>) {
        let mut reversed_keys = Vec::new();
        let mut reversed_values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek_to_last();
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            reversed_keys.push(iter.key().unwrap().to_vec());
            reversed_values.push(iter.value().unwrap().to_vec());
            iter.prev();
            count += 1;
        }
        reversed_keys.reverse();
        reversed_values.reverse();

        (reversed_keys, reversed_values)
    }

    #[inline]
    pub fn get_rows_between(
        &self,
        begin_key: &Key,
        end_key: &Key,
        limit: u32,
    ) -> (Vec<Key>, Vec<Value>) {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek(begin_key);
        let end_key_ref: &[u8] = end_key.as_ref();
        while iter.is_valid() {
            if count >= limit {
                break;
            }

            let key = iter.key().unwrap();
            if key > end_key_ref {
                break;
            }
            keys.push(key.to_vec());
            values.push(iter.value().unwrap().to_vec());

            iter.next();
            count += 1;
        }

        (keys, values)
    }

    #[inline]
    pub fn get_first_key(&self) -> Option<Key> {
        let mut iter = self.inner.iter();
        iter.seek_to_first();
        if iter.is_valid() {
            Some(iter.key().unwrap().to_vec())
        } else {
            None
        }
    }

    #[inline]
    pub fn get_last_key(&self) -> Option<Key> {
        let mut iter = self.inner.iter();
        iter.seek_to_last();
        if iter.is_valid() {
            Some(iter.key().unwrap().to_vec())
        } else {
            None
        }
    }

    #[inline]
    pub fn get_boundary_keys(&self) -> Option<(Key, Key)> {
        let mut iter = self.inner.iter();
        iter.seek_to_first();
        if iter.is_valid() {
            let first_key = iter.key().unwrap().to_vec();
            iter.seek_to_last();
            let last_key = iter.key().unwrap().to_vec();
            Some((first_key, last_key))
        } else {
            None
        }
    }

    #[inline]
    pub fn get_value(&self, key: &Key) -> Option<Value> {
        match self.inner.get(key).unwrap() {
            Some(value) => Some(value.to_vec()),
            None => None,
        }
    }

    #[inline]
    pub fn get_nth_last_value(&self, n: u32) -> Option<Value> {
        let mut value = None;
        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek_to_last();
        while iter.is_valid() {
            if count == n {
                value = Some(iter.value().unwrap().to_vec());
                break;
            }
            iter.prev();
            count += 1;
        }
        value
    }

    #[inline]
    pub fn get_values_since(&self, key: &Key, limit: u32) -> Vec<Value> {
        let mut values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek(key);
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            values.push(iter.value().unwrap().to_vec());
            iter.next();
            count += 1;
        }

        values
    }

    #[inline]
    pub fn get_values_until(&self, key: &Key, limit: u32) -> Vec<Value> {
        let mut reversed_values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek_for_prev(key);
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            reversed_values.push(iter.value().unwrap().to_vec());
            iter.prev();
            count += 1;
        }
        reversed_values.reverse();

        reversed_values
    }

    #[inline]
    pub fn get_values_until_last(&self, limit: u32) -> Vec<Value> {
        let mut reversed_values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek_to_last();
        while iter.is_valid() {
            if count >= limit {
                break;
            }
            reversed_values.push(iter.value().unwrap().to_vec());
            iter.prev();
            count += 1;
        }
        reversed_values.reverse();

        reversed_values
    }

    #[inline]
    pub fn get_values_between(&self, begin_key: &Key, end_key: &Key, limit: u32) -> Vec<Value> {
        let mut values = Vec::new();

        let mut count = 0;
        let mut iter = self.inner.iter();
        iter.seek(begin_key);
        let end_key_ref: &[u8] = end_key.as_ref();
        while iter.is_valid() {
            if count >= limit {
                break;
            }

            let key = iter.key().unwrap();
            if key > end_key_ref {
                break;
            }
            values.push(iter.value().unwrap().to_vec());

            iter.next();
            count += 1;
        }

        values
    }
}
