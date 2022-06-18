# ruby-lsp-that-works

The hypothesis of the whole project is that we can leverage modern tools to be provide a better LSP for ruby.
There is solargraph, which is a great start but it falls short or outright doesn't work in some cases.

I'm really not sure if the ideas I have about the LSP implemenation are sane, but that's exactly what I want to figure out.

So this project is an experiment to build a better, as in features that work more often an more reliably, LSP that has no dependency to ruby
and uses modern tools like:

* tree-sitter
* mruby (potentially required, but I don't know yet)
* rust 
