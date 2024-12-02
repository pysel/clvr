.PHONY: subdate

update-hook-abi:
	(cd clvr-contracts; forge build)
	jq .abi clvr-contracts/out/ClvrHook.sol/ClvrHook.json > clvr-api/abis/ClvrHook.json

subdate:
	git add -A
	git commit -am "Update submodules"
	git push
