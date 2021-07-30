use std::cmp;

pub(crate) fn lev_distance(me: &str, t: &str) -> usize {
    if me.is_empty() {
        return t.chars().count();
    }
    if t.is_empty() {
        return me.chars().count();
    }

    let mut dcol = (0..=t.len()).collect::<Vec<_>>();
    let mut t_last = 0;

    for (i, mc) in me.chars().enumerate() {
        let mut current = i;
        dcol[0] = current + 1;

        for (j, tc) in t.chars().enumerate() {
            let next = dcol[j + 1];

            dcol[j + 1] = cmp::min(current, next);

            if mc != tc {
                dcol[j + 1] = cmp::min(dcol[j + 1], dcol[j]) + 1;
            }

            current = next;
            t_last = j;
        }
    }

    dcol[t_last + 1]
}

pub(crate) fn closest<T, I, F>(search: &str, iter: I, to_key: F) -> Option<T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> &String,
{
    iter.map(|e| (lev_distance(search, to_key(&e)), e))
        .filter(|&(d, _)| d < 4)
        .min_by_key(|t| t.0)
        .map(|t| t.1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_probable_typos() {
        assert_eq!(lev_distance("Click me", "Click me!"), 1);

        let element_text_content = "Click Me!".to_owned();

        closest("Clik Me", [element_text_content].iter(), |s| s)
            .expect("'Clik Me' to find 'Click Me!' as a recommendation");
    }
}
