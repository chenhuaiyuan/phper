// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use crate::{
    errors::HttpClientError,
    request::REQUEST_BUILDER_CLASS_NAME,
    utils::{replace_and_get, replace_and_set},
};
use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntry, StatefulClass, Visibility},
    functions::Argument,
};
use reqwest::blocking::{Client, ClientBuilder};
use std::time::Duration;

const HTTP_CLIENT_BUILDER_CLASS_NAME: &str = "HttpClient\\HttpClientBuilder";
const HTTP_CLIENT_CLASS_NAME: &str = "HttpClient\\HttpClient";

pub fn make_client_builder_class() -> StatefulClass<ClientBuilder> {
    let mut class = StatefulClass::new_with_default_state(HTTP_CLIENT_BUILDER_CLASS_NAME);

    class.add_method(
        "timeout",
        Visibility::Public,
        |this, arguments| {
            let ms = arguments[0].expect_long()?;
            let state: &mut ClientBuilder = this.as_mut_state();
            replace_and_set(state, |builder| {
                builder.timeout(Duration::from_millis(ms as u64))
            });
            Ok::<_, HttpClientError>(this.to_ref_owned())
        },
        vec![Argument::by_val("ms")],
    );

    class.add_method(
        "cookie_store",
        Visibility::Public,
        |this, arguments| {
            let enable = arguments[0].expect_bool()?;
            let state: &mut ClientBuilder = this.as_mut_state();
            replace_and_set(state, |builder| builder.cookie_store(enable));
            Ok::<_, HttpClientError>(this.to_ref_owned())
        },
        vec![Argument::by_val("enable")],
    );

    class.add_method(
        "build",
        Visibility::Public,
        |this, _arguments| {
            let state = this.as_mut_state();
            let client = replace_and_get(state, ClientBuilder::build)?;
            let mut object = ClassEntry::from_globals(HTTP_CLIENT_CLASS_NAME)?.init_object()?;
            unsafe {
                *object.as_mut_state() = Some(client);
            }
            Ok::<_, HttpClientError>(object)
        },
        vec![],
    );

    class
}

pub fn make_client_class() -> StatefulClass<Option<Client>> {
    let mut class = StatefulClass::<Option<Client>>::new_with_default_state(HTTP_CLIENT_CLASS_NAME);

    class.add_method("__construct", Visibility::Private, |_, _| {}, vec![]);

    class.add_method(
        "get",
        Visibility::Public,
        |this, arguments| {
            let url = arguments[0].as_z_str().unwrap().to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.get(url);
            let mut object = ClassEntry::from_globals(REQUEST_BUILDER_CLASS_NAME)?.init_object()?;
            unsafe {
                *object.as_mut_state() = Some(request_builder);
            }
            Ok::<_, HttpClientError>(object)
        },
        vec![Argument::by_val("url")],
    );

    class.add_method(
        "post",
        Visibility::Public,
        |this, arguments| {
            let url = arguments[0].as_z_str().unwrap().to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.post(url);
            let mut object = ClassEntry::from_globals(REQUEST_BUILDER_CLASS_NAME)?.init_object()?;
            unsafe {
                *object.as_mut_state() = Some(request_builder);
            }
            Ok::<_, HttpClientError>(object)
        },
        vec![Argument::by_val("url")],
    );

    class
}
