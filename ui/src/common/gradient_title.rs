use stylist::style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GradientTitleProps {
    pub children: Children,
}

#[function_component(GradientTitle)]
pub fn gradient_title(props: &GradientTitleProps) -> Html {
    let style = style!(
        r"
        display: flex;
        margin-bottom: 21px;

        .gradient-title__content {
            font-size: 29px;
            position: relative;
        }

        .gradient-title__content::before {
            content: '';
            opacity: 0.7;
            width: 120%;
            display: block;
            height: 21px;
            border-radius: 100px;
            background: linear-gradient(90deg, #91EAE4 0%, #86A8E7 22%, #7F7FD5 100%);
            position: absolute;
            z-index: -1;
            bottom: 0px;
            left: -5%;
        }
    "
    )
    .unwrap();

    html! {
        <div class={style}>
            <div class="gradient-title__content">
                { props.children.clone() }
            </div>
        </div>
    }
}
