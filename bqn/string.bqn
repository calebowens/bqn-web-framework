string ← {
	# Original
	# SplitPositions ← {⟨-1⟩∾(⟨⟩ {𝕨≠0?⟨𝕨⟩∾𝕩;𝕩}´ (↕∘≠×((1≠⊣)¨𝕨⊐⊣)) 𝕩)∾⟨≠𝕩⟩}

	# SplitAt ← {(𝕩⊏˜1+⊑+↕∘(1⊑⊣-1+⊑))¨ <˘ 2↕(𝕨 SplitPositions 𝕩)}

	SplitAll ⇐ {(𝕩⊏˜⊑+↕∘-∘¬´)¨<˘2↕(/1∾˜1∾𝕨=𝕩)}

	# Not a fan of this function
	Split ⇐ {
		x ← 𝕩
		w ← 𝕨
		split_point ← ⊑𝕩⊐𝕨

		{ split_point=(≠x) ? ⟨x, ⟨⟩⟩ ; (x⊏˜⊣)¨(↕⋈+⟜1+↕∘((¯1+≠x)-⊣))split_point }
	}
}
