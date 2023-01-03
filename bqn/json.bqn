json ← {
	IsChar ⇐ {
		¬IsArray 𝕩 ?
			O ← {-𝕩}⎊0
			0=O𝕩 ;
			0
	}

	IsArray ⇐ {0<=𝕩}

	ArrayIsString ⇐ {0=+´¬∘IsChar¨𝕩}

	StringifyArray ⇐ {
		"[" ∾ ({𝕨 ∾ ", " ∾ 𝕩}´Stringify¨𝕩) ∾ "]"
	}

	Stringify ⇐ {
		x ← 𝕩

		{ 
			IsArray x ?
				{ ArrayIsString x ? """" ∾ x ∾ """" ; StringifyArray x } ;
				IsChar x ?  """" ∾ x ∾ """" ; •Fmt x
		}
	}
}
