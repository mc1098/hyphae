use std::marker::PhantomData;

use wasm_bindgen::JsCast;
use web_sys::NodeList;

/// Iterator for [`Element`](web_sys::Element)s
pub struct ElementIter<'a, T: JsCast> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
    _marker: PhantomData<&'a T>,
}

#[allow(dead_code)]
impl<T: JsCast> ElementIter<'_, T> {
    pub(crate) fn new(node_list: Option<NodeList>) -> Self {
        if let Some(node_list) = node_list {
            node_list.into()
        } else {
            Self {
                iter: Box::new(std::iter::empty()),
                _marker: PhantomData,
            }
        }
    }
}

impl<T: JsCast> From<NodeList> for ElementIter<'_, T> {
    fn from(node_list: NodeList) -> Self {
        let mut nodes = vec![];
        for i in 0..node_list.length() {
            if let Some(element) = node_list.get(i).and_then(|node| node.dyn_into().ok()) {
                nodes.push(element);
            }
        }

        Self {
            iter: Box::new(nodes.into_iter()),
            _marker: PhantomData,
        }
    }
}

impl<T: JsCast> Iterator for ElementIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Iterator for [`NodeList`]
pub(crate) struct RawNodeListIter<T> {
    index: u32,
    node_list: Option<NodeList>,
    _marker: PhantomData<T>,
}

impl<T> RawNodeListIter<T>
where
    T: JsCast,
{
    pub(crate) fn new(node_list: Option<NodeList>) -> Self {
        Self {
            index: 0,
            node_list,
            _marker: PhantomData,
        }
    }
}

impl<T> Iterator for RawNodeListIter<T>
where
    T: JsCast,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_list = self.node_list.as_ref()?;

        loop {
            if self.index < node_list.length() {
                let node = node_list.get(self.index)?;
                self.index += 1;
                if let Ok(value) = node.dyn_into() {
                    break Some(value);
                }
            } else {
                break None;
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            self.node_list.as_ref().map(|list| list.length() as usize),
        )
    }
}
