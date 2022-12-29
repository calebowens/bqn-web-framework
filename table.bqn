table ← {
	Keys ⇐ ⊑¨
	Values ⇐ (1⊑⊣)¨
	Get ⇐ { 
			key ← ⟨𝕨⟩ ⊐˜ Keys 𝕩

			(≠𝕩)≠(⊑key) ? key ⊑ Values 𝕩 ; ⟨⟩
		}
}
