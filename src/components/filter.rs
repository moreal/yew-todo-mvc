use crate::domains::filter::Filter;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FilterEntryProps {
    pub filter: Filter,
    pub selected: bool,
    pub onsetfilter: Callback<Filter>,
}

#[function_component(FilterEntry)]
pub fn filter(props: &FilterEntryProps) -> Html {
    let filter_string = props.filter.to_string();
    let setfilter = {
        let onsetfilter = props.onsetfilter.clone();
        let filter = props.filter;
        move |_| onsetfilter.emit(filter)
    };

    html! { <li><a class={ if props.selected {"selected"} else {""} } onclick={setfilter}>{ filter_string }</a></li> }
}
