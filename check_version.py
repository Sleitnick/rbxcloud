import sys

def main():
	if len(sys.argv) <= 1:
		print("missing version argument", file=sys.stderr)
		exit(1)
	
	version = sys.argv[1]
	cargo_version = ""
	found_cargo_version = False
	matched_version = False

	# Find first occurrence of "version = x.y.z" and assume that as the project version:
	with open("Cargo.toml", mode="r") as cargo_file:
		lines = cargo_file.readlines()
		for line in lines:
			pair = line[:-1].split(sep="=")
			if len(pair) == 2 and pair[0].strip() == "version":
				cargo_version = "v" + pair[1].strip()[1:-1]
				matched_version = cargo_version == version
				found_cargo_version = True
				break
	
	if not found_cargo_version:
		print("failed to find cargo version", file=sys.stderr)
		exit(1)

	if not matched_version:
		print(f"version mismatch (input: {version} | cargo: {cargo_version})", file=sys.stderr)
		exit(1)
	
	print("versions matched")

if __name__ == "__main__":
	main()
