query_params ← •Import "bqn/query_params.bqn"
table ← •Import "bqn/table.bqn"
router ← •Import "bqn/router.bqn"
⟨InternalError⟩ ← •Import "bqn/errors.bqn"

controller ← {
	Greet ⇐ {
		name ← "name" table.Get query_params.BasicParse 𝕩.uri.query

		name≢⟨⟩ ?
			⟨
				200
				⟨⟨"Content-Type", "text/html"⟩⟩
				"Hello " ∾ name
			⟩ ;
			⟨
				302
				⟨⟨"Location", "/"⟩⟩
				""
			⟩
	}

	home ⇐ ⟨
		200
		⟨⟨"Content-Type", "text/html"⟩⟩
		"Visit /greet with query param name to be greeted"
	⟩

	Throws ⇐ {𝕊
		⟨"Something went wrong!", 20⟩ ! 0
	}
}

controller.Home router._get_ "/"
controller.Greet router._get_ "/greet"
controller.Throws router._get_ "/throws"

{
	router.HandleRequest⎊{InternalError •CurrentError} 𝕩
}
