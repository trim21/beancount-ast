import abc

from beancount_ast._ast import (
    Balance,
    Close,
    Comment,
    Commodity,
    Custom,
    Document,
    Event,
    File,
    Headline,
    Include,
    Note,
    Open,
    Option,
    Pad,
    Plugin,
    PopMeta,
    Price,
    PushMeta,
    Query,
    Span,
    Tag,
    Transaction,
)


class Directive(abc.ABC):
    @property
    def span(self) -> Span: ...
    @property
    def file(self) -> File: ...
    def dump(self) -> str: ...


Directive.register(Open)
Directive.register(Close)
Directive.register(Balance)
Directive.register(Pad)
Directive.register(Transaction)
Directive.register(Commodity)
Directive.register(Price)
Directive.register(Event)
Directive.register(Query)
Directive.register(Note)
Directive.register(Document)
Directive.register(Custom)
Directive.register(Option)
Directive.register(Include)
Directive.register(Plugin)
Directive.register(Tag)
Directive.register(PushMeta)
Directive.register(PopMeta)
Directive.register(Comment)
Directive.register(Headline)
