string ← •Import "string.bqn"

query_params ← {
	BasicParse ⇐ {
		('='string.Split⊣)¨('&' string.SplitAll 𝕩)
	}
}
