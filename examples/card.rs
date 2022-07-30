//! 3D rendering
//!
//! This examples shows a simple graphics application with Dioxus UI rendered on top of it

use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch_cfg(app, |cfg| {
        cfg
            .with_window(|w|
                w
                    .with_title("3D rendering demo")
                    .with_decorations(true)
            )
            .with_custom_head(
                "<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()
            )
    });
}

fn app(cx: Scope) -> Element {


    cx.render(rsx! {
        Child {}

        // div {
        //     class: "bg-white dark:bg-gray-900",
        //     div {
        //         class: "flex justify-center h-screen",
        //         div {
        //             class: "hidden bg-cover lg:block lg:w-2/3",
        //             style: "background-image: url(https://images.unsplash.com/photo-1616763355603-9755a640a287?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80)",
        //             div {
        //                 class: "flex items-center h-full px-20 bg-gray-900 bg-opacity-40",
        //                 div {
        //                     h2 {
        //                         class: "text-4xl font-bold text-white",
        //                         "Brand"
        //                     }
        //
        //                     p {
        //                         class: "max-w-xl mt-3 text-gray-300",
        //                         "Lorem ipsum dolor sit, amet consectetur adipisicing elit. In autem ipsa, nulla laboriosam dolores, repellendus perferendis libero suscipit nam temporibus molestiae"
        //                     }
        //                 }
        //             }
        //         }
        //
        //         div {
        //             class: "flex items-center w-full max-w-md px-6 mx-auto lg:w-2/6",
        //             div {
        //                 class: "flex-1",
        //                 div {
        //                     class: "text-center",
        //
        //                     h2 {
        //                         class: "text-4xl font-bold text-center text-gray-700 dark:text-white",
        //                         "Brand"
        //                     }
        //
        //                     p {
        //                         class: "mt-3 text-gray-500 dark:text-gray-300",
        //                         "Sign in to access your account"
        //                     }
        //                 }
        //
        //                 div {
        //                     class: "mt-8",
        //                     form {
        //                         div {
        //                             label {
        //                                 r#for: "email",
        //                                 class: "block mb-2 text-sm text-gray-600 dark:text-gray-200",
        //                                 "Email address"
        //                             }
        //
        //                             input {
        //                                 r#type: "email",
        //                                 name: "email",
        //                                 id: "email",
        //                                 placeholder: "example@example.com",
        //                                 class: "block w-full px-4 py-2 mt-2 text-gray-700 placeholder-gray-400 bg-white border border-gray-200 rounded-md dark:placeholder-gray-600 dark:bg-gray-900 dark:text-gray-300 dark:border-gray-700 focus:border-blue-400 dark:focus:border-blue-400 focus:outline-none focus:ring-opacity-40"
        //                             }
        //                         }
        //
        //                         div {
        //                             class: "mt-6",
        //                             div {
        //                                 class: "flex justify-between mb-2",
        //                                 label {
        //                                     r#for: "password",
        //                                     class: "text-sm text-gray-600 dark:text-gray-200",
        //                                     "Password"
        //                                 }
        //
        //                                 a {
        //                                     href: "#",
        //                                     class: "text-sm text-gray-400 focus:text-blue-500 hover:text-blue-500 hover:underline",
        //                                     "Forgot password?"
        //                                 }
        //                             }
        //
        //                             input {
        //                                 r#type: "password",
        //                                 name: "password",
        //                                 id: "password",
        //                                 placeholder: "Your password",
        //                                 class: "block w-full px-4 py-2 mt-2 text-gray-700 placeholder-gray-400 bg-white border border-gray-200 rounded-md dark:placeholder-gray-600 dark:bg-gray-900 dark:text-gray-300 dark:border-gray-700 dark:focus:border-blue-400 focus:ring-blue-400 focus:outline-none focus:ring focus:ring-opacity-40"
        //                             }
        //                         }
        //
        //                         div {
        //                             class: "mt-6",
        //                             button {
        //                                 class: "w-full px-4 py-2 tracking-wide text-white transition-colors duration-200 transform bg-blue-500 rounded-md hover:bg-blue-400 focus:outline-none focus:bg-blue-400 focus:ring focus:ring-blue-300 focus:ring-opacity-50",
        //                                 "Sign in"
        //                             }
        //                         }
        //                     }
        //
        //                     p {
        //                         class: "mt-6 text-sm text-center text-gray-400",
        //                         "Don't have an account yet? ",
        //                         a {
        //                             href: "#",
        //                             class: "text-blue-500 focus:outline-none focus:underline hover:underline",
        //                             "Sign up"
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    })
}

fn Child(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex h-screen items-center justify-center px-4",
            div {
                class: "max-w-sm overflow-hidden rounded-xl bg-white shadow-md duration-200 hover:scale-105 hover:shadow-xl",
                img {
                    class: "h-auto w-full",
                    alt: "plant",
                    src: "https://i.imgur.com/5dmBrx6.jpg"
                }

                div {
                    class: "p-5",
                    p {
                        class: "text-medium mb-5 text-gray-700",
                        "Lorem ipsum dolor sit, amet consectetur adipisicing elit. In autem ipsa, nulla laboriosam dolores, repellendus perferendis libero suscipit nam temporibus molestiae."
                    }

                    button {

                        class: "w-full rounded-md bg-indigo-600 py-2 text-indigo-100 hover:bg-indigo-500 hover:shadow-md duration-75",
                        "Click Me"
                    }
                }
            }
        }
    })
}