import fs from 'fs'

const TOTAL_SIZE = 70_000_000
const SPACE_NEEDED = 30_000_000

type Directory = {
  type: 'dir',
  name: string
  parent?: Directory
  contents: (Directory | File)[]
}

type File = {
  type: 'file',
  name: string
  parent: Directory
  size: number
}

// Calculate the recursive size of a directory
const dirSize = (dir: Directory): number => dir.contents
  .map(i => i.type === 'file' ? i.size : dirSize(i))
  .reduce((total, size) => total + size, 0)

const root: Directory = {
  type: 'dir',
  name: '/',
  parent: undefined,
  contents: [],
}

const input = fs.readFileSync('input.txt', 'utf8')

// Keep track of the current directory
let cd = root
// Keep track of every directory
const allDirs: Directory[] = [root]

// Parse input
input.trim().split('\n').forEach(line => {
  if (line == '$ cd ..' && cd.parent !== undefined) {
    // Move up
    cd = cd.parent
  } else if (line == '$ cd /') {
    // Move to root
    cd = root
  } else if (line.startsWith('$ cd ')) {
    // Move to folder
    cd = cd.contents.find(d => d.name === line.replace('$ cd ', '')) as Directory
  } else if (line !== '$ ls') {
    if (line.startsWith('dir')) {
      // New dir found
      const name = line.replace('dir ', '')
      if (!cd.contents.some(d => d.name === name)) {
        const dir: Directory = { type: 'dir', name, parent: cd, contents: [] }
        cd.contents.push(dir)
        allDirs.push(dir)
      }
    } else {
      // New file found
      const [size, name] = line.split(' ')
      if (!cd.contents.some(d => d.name === name)) {
        cd.contents.push({ type: 'file', name, parent: cd, size: Number(size) })
      }
    }
  }
})

// Calculate the total size of all directories at most 100000
const dirSizes = allDirs
  .map(d => ({ ...d, size: dirSize(d) }))

const size = dirSizes
  .filter(d => d.size <= 100000)
  .reduce((total, d) => total + d.size, 0)

console.log(`The sum of all folder sizes at most 100000 is: ${size}`)

// Calculate how much needs to be deleted to have enough space
const clearAtLeast = SPACE_NEEDED - (TOTAL_SIZE - dirSize(root))

const smallestDirToDelete = dirSizes.filter(d => d.size >= clearAtLeast).sort((a, b) => a.size - b.size)[0]

console.log(`The size of the smallest dir that should be deleted to make room is: ${smallestDirToDelete.size}`)
