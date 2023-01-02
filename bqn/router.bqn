table ← •Import "table.bqn"
⟨_find_⟩ ← •Import "find.bqn"
⟨not_found⟩ ← •Import "errors.bqn"

router ← {
	gets ⇐ ⟨⟩
	puts ⇐ ⟨⟩
	posts ⇐ ⟨⟩
	deletes ⇐ ⟨⟩
	connects ⇐ ⟨⟩
	heads ⇐ ⟨⟩
	optionses ⇐ ⟨⟩
	traces ⇐ ⟨⟩
	patches ⇐ ⟨⟩

	_get_ ⇐ {gets ↩ gets ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_put_ ⇐ {puts ↩ puts ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_post_ ⇐ {posts ↩ posts ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_delete_ ⇐ {deletes ↩ deletes ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_connect_ ⇐ {connects ↩ connects ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_head_ ⇐ {heads ↩ heads ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_options_ ⇐ {optionses ↩ optionses ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_trace_ ⇐ {traces ↩ traces ∾ ⟨𝔽 _mk_route_ 𝕘⟩}
	_patch_ ⇐ {patches ↩ patches ∾ ⟨𝔽 _mk_route_ 𝕘⟩}

	_mk_route_ ← {
		Handler ← 𝔽
		route ← 𝕘

		{
			Run ⇐ Handler
			Matches ⇐ { 𝕩≡route }
		}
	}

	HandleRequest ⇐ {
		x ← 𝕩
		routes ← RouteMapFor x.method
		
		0≠≠routes ? {
			Route ← {𝕩.Matches x.uri.path} _find_ routes
			route≢⟨⟩ ? route.Run x ; not_found 
		} ; not_found
	}

	RouteMapFor ← {
		m ← 𝕩
		{
			m≡"GET" ? gets ;
			m≡"PUT" ? puts ;
			m≡"POST" ? posts ;
			m≡"DELETE" ? deletes ;
			m≡"CONNECT" ? connects ;
			m≡"HEAD" ? heads ;
			m≡"OPTIONS" ? optionses ;
			m≡"TRACE" ? traces ;
			m≡"PATCH" ? patches ; ⟨⟩
		}
	}
}
