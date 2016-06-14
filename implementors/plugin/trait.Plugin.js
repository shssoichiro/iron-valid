(function() {var implementors = {};
implementors["persistent"] = ["impl&lt;'a,&nbsp;'b,&nbsp;P:&nbsp;<a class='trait' href='typemap/trait.Key.html' title='typemap::Key'>Key</a>&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='persistent/struct.State.html' title='persistent::State'>State</a>&lt;P&gt; <span class='where'>where P::Value: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a></span>","impl&lt;'a,&nbsp;'b,&nbsp;P:&nbsp;<a class='trait' href='typemap/trait.Key.html' title='typemap::Key'>Key</a>&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='persistent/struct.Read.html' title='persistent::Read'>Read</a>&lt;P&gt; <span class='where'>where P::Value: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html' title='core::marker::Sync'>Sync</a></span>","impl&lt;'a,&nbsp;'b,&nbsp;P:&nbsp;<a class='trait' href='typemap/trait.Key.html' title='typemap::Key'>Key</a>&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='persistent/struct.Write.html' title='persistent::Write'>Write</a>&lt;P&gt; <span class='where'>where P::Value: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a></span>",];implementors["bodyparser"] = ["impl&lt;'a,&nbsp;'b&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='bodyparser/struct.Raw.html' title='bodyparser::Raw'>Raw</a>","impl&lt;'a,&nbsp;'b&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='bodyparser/struct.Json.html' title='bodyparser::Json'>Json</a>","impl&lt;'a,&nbsp;'b,&nbsp;T&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='bodyparser/struct.Struct.html' title='bodyparser::Struct'>Struct</a>&lt;T&gt; <span class='where'>where T: <a class='trait' href='serde/de/trait.Deserialize.html' title='serde::de::Deserialize'>Deserialize</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a></span>",];implementors["urlencoded"] = ["impl&lt;'a,&nbsp;'b&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='urlencoded/struct.UrlEncodedQuery.html' title='urlencoded::UrlEncodedQuery'>UrlEncodedQuery</a>","impl&lt;'a,&nbsp;'b&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='urlencoded/struct.UrlEncodedBody.html' title='urlencoded::UrlEncodedBody'>UrlEncodedBody</a>",];implementors["params"] = ["impl&lt;'a,&nbsp;'b&gt; <a class='trait' href='plugin/trait.Plugin.html' title='plugin::Plugin'>Plugin</a>&lt;<a class='struct' href='iron/request/struct.Request.html' title='iron::request::Request'>Request</a>&lt;'a,&nbsp;'b&gt;&gt; for <a class='struct' href='params/struct.Params.html' title='params::Params'>Params</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()