{
	⟨method⟩ ← 𝕩

	•Out "processing request"

	⟨200, ⟨⟨"Content-Type", "application/json"⟩⟩, "{""methodWas"": """ ∾ method ∾ """ }"⟩
}
