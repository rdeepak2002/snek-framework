class Foo:
    def render(self):
        x = 12
        y = 5.5
        return f"<div>" \
               f"<h1>Header</h1>" \
               f"<p>foo bar {x + y}</p>" \
               f"</div>"


Foo()
