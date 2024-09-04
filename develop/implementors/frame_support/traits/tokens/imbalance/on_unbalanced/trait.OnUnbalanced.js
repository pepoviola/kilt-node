(function() {var implementors = {
"runtime_common":[["impl&lt;R, Ratio, Beneficiary1, Beneficiary2&gt; OnUnbalanced&lt;Imbalance&lt;&lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Inspect&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::Balance, &lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::OnDropCredit, &lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::OnDropDebt&gt;&gt; for <a class=\"struct\" href=\"runtime_common/fees/struct.SplitFeesByRatio.html\" title=\"struct runtime_common::fees::SplitFeesByRatio\">SplitFeesByRatio</a>&lt;R, Ratio, Beneficiary1, Beneficiary2&gt;<span class=\"where fmt-newline\">where\n    R: Config,\n    Beneficiary1: OnUnbalanced&lt;Credit&lt;&lt;R as Config&gt;::AccountId, Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt;&gt;&gt;,\n    Beneficiary2: OnUnbalanced&lt;Credit&lt;&lt;R as Config&gt;::AccountId, Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt;&gt;&gt;,\n    Ratio: Get&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u32.html\">u32</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u32.html\">u32</a>)&gt;,</span>"],["impl&lt;T&gt; OnUnbalanced&lt;Imbalance&lt;&lt;Pallet&lt;T, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Inspect&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::Balance, &lt;Pallet&lt;T, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::OnDropCredit, &lt;Pallet&lt;T, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;T as Config&gt;::AccountId&gt;&gt;::OnDropDebt&gt;&gt; for <a class=\"struct\" href=\"runtime_common/struct.SendDustAndFeesToTreasury.html\" title=\"struct runtime_common::SendDustAndFeesToTreasury\">SendDustAndFeesToTreasury</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Config + Config,</span>"],["impl&lt;R&gt; OnUnbalanced&lt;&lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Currency&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::NegativeImbalance&gt; for <a class=\"struct\" href=\"runtime_common/fees/struct.ToAuthor.html\" title=\"struct runtime_common::fees::ToAuthor\">ToAuthor</a>&lt;R&gt;<span class=\"where fmt-newline\">where\n    R: Config + Config,\n    &lt;R as Config&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"type\" href=\"runtime_common/type.AccountId.html\" title=\"type runtime_common::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"runtime_common/type.AccountId.html\" title=\"type runtime_common::AccountId\">AccountId</a>&gt;,\n    &lt;R as Config&gt;::Balance: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u128.html\">u128</a>&gt;,</span>"],["impl&lt;R&gt; OnUnbalanced&lt;Imbalance&lt;&lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Inspect&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::Balance, &lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::OnDropCredit, &lt;Pallet&lt;R, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.unit.html\">()</a>&gt; as Balanced&lt;&lt;R as Config&gt;::AccountId&gt;&gt;::OnDropDebt&gt;&gt; for <a class=\"struct\" href=\"runtime_common/fees/struct.ToAuthorCredit.html\" title=\"struct runtime_common::fees::ToAuthorCredit\">ToAuthorCredit</a>&lt;R&gt;<span class=\"where fmt-newline\">where\n    R: Config + Config,\n    &lt;R as Config&gt;::AccountId: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"type\" href=\"runtime_common/type.AccountId.html\" title=\"type runtime_common::AccountId\">AccountId</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"type\" href=\"runtime_common/type.AccountId.html\" title=\"type runtime_common::AccountId\">AccountId</a>&gt;,\n    &lt;R as Config&gt;::Balance: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.74.0/std/primitive.u128.html\">u128</a>&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()