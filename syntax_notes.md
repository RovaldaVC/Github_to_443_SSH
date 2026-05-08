### -- Import part -- ###
LazyLock use is to Compile once and use as much as you want.

### -- get_remote fn -- ###
.ok()? turns Result<Output, Error> into: Option<T> .
get_remote gives us the url of our remote repository.


### -- convert_to_443 fn -- ###
in Static RE:LazyLock<Regex> we create a structure, (.+/.+) can be replaced with whatever we give to it. next part.
in caps we gave (.+/.+) our url.
in repo get(1) will returm (.+/.+) which means (user/repo)
repo has user/repo inside it!
at last we will get the url based on 443.


### -- Extra Stuff -- ###
static means Global use, wherever.
can't use static RE:Regex, static needs a compile-time type validation. but Regex is not a compile time fn, it's runtime fn! key solution is: when we say Lazy<Regex> it will become like a struct.

Lazy<Regex> is like: struct Lazy{value: Maybe<Regex>}
