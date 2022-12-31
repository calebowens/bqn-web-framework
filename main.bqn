query_params ← •Import "bqn/query_params.bqn"
table ← •Import "bqn/table.bqn"

{
	name ← "name" table.Get query_params.BasicParse 𝕩.uri.query
	phrase ← { name≢⟨⟩ ? "Hello " ∾ name ; "Hello world" }

	⟨
		200
		⟨⟨"Content-Type", "text/html"⟩⟩
		phrase
	⟩
}
