//! Books cover wall: order books into reading-now, year and undated groups.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupLabel {
    ReadingNow,
    Year(u32),
    Earlier,
}

/// One heading on the wall and the indices (into the input) of its books.
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    pub label: GroupLabel,
    pub indices: Vec<usize>,
}

/// Group books given as `(is_reading, read_year)`: reading first, then read
/// years newest first, then undated books. Input order is kept within a group;
/// empty groups are left out.
pub fn group(items: &[(bool, Option<u32>)]) -> Vec<Group> {
    let mut years: Vec<u32> = items.iter().filter(|(r, _)| !r).filter_map(|(_, y)| *y).collect();
    years.sort_unstable_by(|a, b| b.cmp(a));
    years.dedup();

    let mut labels = vec![GroupLabel::ReadingNow];
    labels.extend(years.into_iter().map(GroupLabel::Year));
    labels.push(GroupLabel::Earlier);

    labels
        .into_iter()
        .map(|label| {
            let indices = items
                .iter()
                .enumerate()
                .filter(|(_, (reading, year))| match label {
                    GroupLabel::ReadingNow => *reading,
                    GroupLabel::Year(y) => !reading && *year == Some(y),
                    GroupLabel::Earlier => !reading && year.is_none(),
                })
                .map(|(i, _)| i)
                .collect();
            Group { label, indices }
        })
        .filter(|g| !g.indices.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_in_order() {
        let g = group(&[(false, Some(2024)), (true, None), (false, None), (false, Some(2026)), (false, Some(2024))]);
        assert_eq!(
            g.iter().map(|g| g.label).collect::<Vec<_>>(),
            vec![GroupLabel::ReadingNow, GroupLabel::Year(2026), GroupLabel::Year(2024), GroupLabel::Earlier]
        );
        assert_eq!(g[2].indices, vec![0, 4]);
    }

    #[test]
    fn no_reading_group_when_empty() {
        assert_eq!(group(&[(false, Some(2025))])[0].label, GroupLabel::Year(2025));
        assert!(group(&[]).is_empty());
    }
}
