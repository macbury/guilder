import last from 'lodash/last';
import upperFirst from 'lodash/upperFirst';

export type FilterOptions = {
  name: string,
  sort: string,
  direction: 'desc' | 'asc',
  categories: number[]
}

export function mapSortKey(sort : string) {
  return last(sort.split('.'))
}
