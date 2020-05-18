#![allow(missing_docs)]

use juniper::Context;

use crate::model::{Character, Database, Droid, Episode, Human};

impl Context for Database {}

juniper::graphql_interface!(<'a> &'a dyn Character: Database as "Character" |&self| {
    description: "A character in the Star Wars Trilogy"

    field id() -> &str as "The id of the character" {
        self.id()
    }

    field name() -> Option<&str> as "The name of the character" {
        Some(self.name())
    }

    field friends(&executor) -> Vec<&dyn Character>
    as "The friends of the character" {
        executor.context().get_friends(self.as_character())
    }

    field appears_in() -> &[Episode] as "Which movies they appear in" {
        self.appears_in()
    }

    instance_resolvers: |&context| {
        &dyn Human => context.get_human(&self.id()),
        &dyn Droid => context.get_droid(&self.id()),
    }
});

#[juniper::graphql_object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
    interfaces = [&dyn Character],
    // FIXME: make async work
    noasync
)]
/// A humanoid creature in the Star Wars universe.
impl<'a> &'a dyn Human {
    /// The id of the human
    fn id(&self) -> &str {
        self.id()
    }

    /// The name of the human
    fn name(&self) -> Option<&str> {
        Some(self.name())
    }

    /// The friends of the human
    fn friends(&self, ctx: &Database) -> Vec<&dyn Character> {
        ctx.get_friends(self.as_character())
    }

    /// Which movies they appear in
    fn appears_in(&self) -> &[Episode] {
        self.appears_in()
    }

    /// The home planet of the human
    fn home_planet(&self) -> &Option<String> {
        self.home_planet()
    }
}

#[juniper::graphql_object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
    interfaces = [&dyn Character],
    // FIXME: make async work
    noasync
)]
/// A mechanical creature in the Star Wars universe.
impl<'a> &'a dyn Droid {
    /// The id of the droid
    fn id(&self) -> &str {
        self.id()
    }

    /// The name of the droid
    fn name(&self) -> Option<&str> {
        Some(self.name())
    }

    /// The friends of the droid
    fn friends(&self, ctx: &Database) -> Vec<&dyn Character> {
        ctx.get_friends(self.as_character())
    }

    /// Which movies they appear in
    fn appears_in(&self) -> &[Episode] {
        self.appears_in()
    }

    /// The primary function of the droid
    fn primary_function(&self) -> &Option<String> {
        self.primary_function()
    }
}

#[derive(Debug)]
struct Id(String);

use juniper::{Value, ParseScalarResult, ParseScalarValue};

#[juniper::graphql_scalar(description = "Id")]
impl<S> GraphQLScalar for Id
where
    S: ScalarValue
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_owned())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<Id> {
        v.as_string_value()
        .map(|v| Id(v.to_owned()))
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}


pub struct Query;

#[juniper::graphql_object(
    Context = Database,
    Scalar = juniper::DefaultScalarValue,
    // FIXME: make async work
    noasync
)]
/// The root query object of the schema
impl Query {
    #[graphql(arguments(id(description = "id of the human")))]
    fn human(executor: &Executor, database: &Database, id: String) -> Option<&dyn Human> {

        use juniper::{LookAheadSelection, LookAheadMethods};

        let info: LookAheadSelection<_> = executor.look_ahead();
        println!("{:#?}", info);

        let friends = info.children().iter().find(|c| c.field_name() == "friends").unwrap();

        database.get_human(&id)
    }

    #[graphql(arguments(id(description = "id of the human as an Id type")))]
    fn humanById(executor: &Executor, database: &Database, id: Id) -> Option<&dyn Human> {
        let info = executor.look_ahead();
        println!("{:#?}", info);

        database.get_human(&id.0)
    }

    #[graphql(arguments(id(description = "id of the droid")))]
    fn droid(executor: &Executor, database: &Database, id: String) -> Option<&dyn Droid> {
        let info = executor.look_ahead();
        println!("{:#?}", info);

        database.get_droid(&id)
    }

    #[graphql(arguments(episode(
        description = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode"
    )))]
    fn hero(executor: &Executor, database: &Database, episode: Option<Episode>) -> Option<&dyn Character> {
        let info = executor.look_ahead();
        println!("{:#?}", info);

        Some(database.get_hero(episode).as_character())
    }

    // Gain access to the executor, which allows you to do look aheads, for example for improved database queries.
    fn with_executor(_executor: &Executor) -> bool {
        true
    }
}
