with open("data", "r") as f:
	nig = f.readlines()
	f.close()

existsStr = ""
outStr = ""

for i in range(len(nig)):
	if not nig[i] in existsStr:
		existsStr += nig[i]
		outStr += nig[i]

with open("data2", "w") as f:
	f.write(outStr)
	f.close()