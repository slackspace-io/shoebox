use leptos::prelude::*;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct HeftyData {
    first_name: String,
    last_name: String,
}

#[component]
pub fn FormExample() -> impl IntoView {
    let submit = ServerAction::<VeryImportantFn>::new();

    view! {
      <ActionForm action=submit>
        <input type="text" name="hefty_arg[first_name]" value="leptos"/>
        <input
          type="text"
          name="hefty_arg[last_name]"
          value="closures-everywhere" oninput="this.form.requestSubmit()"
        />
        <input type="submit"/>
      </ActionForm>
    }
}

#[server]
async fn very_important_fn(hefty_arg: HeftyData) -> Result<(), ServerFnError> {
    println!("First Name: {}", hefty_arg.first_name);
    println!("Last Name: {}", hefty_arg.last_name);
    //check if last_name has a space in it yet

    //split last name on space into vec
    let split_name = hefty_arg.last_name.split(',').collect::<Vec<&str>>();
    //if split_name.len() > 1, then there is a space
    if split_name.len() > 1 {
        println!(", detected");
        for name in split_name {
            println!("Name split: {}", name);
        }
    } else {
        println!("No space detected");
    }

    if !hefty_arg.last_name.contains(' ') {
        //if not, add a space
        //split on space
        let split_name = hefty_arg.last_name.split('-').collect::<Vec<&str>>();
        let mut new_name = String::new();
        for name in split_name {
            new_name.push_str(name);
            new_name.push(' ');
        }
        println!("New Name: {}", new_name);
    } else {
        println!("No space yet");
    }
    Ok(())
}
