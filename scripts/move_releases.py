import sys
import os


def main():
	args = sys.argv
	n_args = len(args)
	if n_args <= 1:
		print("no path argument found", file=sys.stderr)
		exit(1)
	
	path = args[1]

	expected = 0
	total = 0

	for child_path in os.listdir(path):
		filepath = os.path.join(path, child_path)
		
		if not os.path.isdir(filepath):
			print(f"{filepath} is not a dir")
			continue

		expected += 1

		release_filepath = os.path.join(filepath, "release.zip")
		if not os.path.isfile(release_filepath):
			print(f"no release.zip file found in {filepath}", file=sys.stderr)
			continue

		target_filepath = f"{filepath}.zip"
		os.rename(release_filepath, target_filepath)

		total += 1
		print(target_filepath)
	
	if total < expected:
		print(f"{total}/{expected} succeeded [{total - expected} failed]", file=sys.stderr)
		exit(1)
	elif total == 0:
		print("no work", file=sys.stderr)
		exit(1)
	
	print("done")


if __name__ == "__main__":
	main()
