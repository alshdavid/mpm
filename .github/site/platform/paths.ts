import * as path from 'node:path'
import * as url from 'node:url'

const filename = url.fileURLToPath(import.meta.url)
const dirname = path.dirname(filename)
const root = path.normalize(path.join(dirname, '..', '..', '..'))

const builder = (...base: string[]): PathBuilder => (...segments: string[]) => path.join(root, ...base, ...segments)

export type PathBuilder = (...segments: string[]) => string

export const Paths = {
  ['~']: builder()(),
  ['~/']: builder(),
  ['~/temp']: builder('temp')(),
  ['~/temp/']: builder('temp'),
  ['~/dist']: builder('dist')(),
  ['~/dist/']: builder('dist'),
  ['~/models']: builder('models')(),
  ['~/models/']: builder('models'),
  ['~/models/:id']: (id: string) => builder('models')(id),
  ['~/models/:id/']: (id: string) => ({
    ['index.yml']: builder('models')(id, 'index.yml')
  }),
}