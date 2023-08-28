class SamplePage:
  def render(self):
    x = 12
    y = 5.5
    return f"<div>" \
           f"<h1>Header</h1>" \
           f"<p>Sample paragraph</p>" \
           f"<p>Sample Python computation {x + y}</p>" \
           f"</div>"


SamplePage()
