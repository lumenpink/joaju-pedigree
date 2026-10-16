use crate::core::Ped;

pub fn find_children<'a>(ped: &'a Ped, id: &str) -> Vec<&'a str> {
    ped.order
        .iter()
        .filter_map(|child_id| ped.members.get(child_id))
        .filter(|child| child.fid.as_deref() == Some(id) || child.mid.as_deref() == Some(id))
        .map(|child| child.id.as_str())
        .collect()
}

pub fn find_parents<'a>(ped: &'a Ped, id: &str) -> (Option<&'a str>, Option<&'a str>) {
    if let Some(ind) = ped.members.get(id) {
        (ind.fid.as_deref(), ind.mid.as_deref())
    } else {
        (None, None)
    }
}
