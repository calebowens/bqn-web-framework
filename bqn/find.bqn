find ← {
	# This will hit recursion limits on large lists
	_find_ ⇐ {
		F ← 𝔽
		g ← 𝕘
		{ g≡⟨⟩ ? ⟨⟩ ; F⊑g ? ⊑g ; F _find_ (1↓g) }
	}
}
