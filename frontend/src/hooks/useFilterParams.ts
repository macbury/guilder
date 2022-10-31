import { useCallback, useMemo } from 'react';
import toNumber from 'lodash/toNumber';
import { useSearchParams } from 'react-router-dom';
import { FilterOptions } from '../store/hooks/filters';

export function useFilterParams() {
  const [searchParams, setSearchParams] = useSearchParams();

  const onSortChange = useCallback((sortColumn : string, sortType : any) => {
    searchParams.set('direction', sortType);
    searchParams.set('sort', sortColumn);
    setSearchParams(searchParams, { replace: false });
  }, [setSearchParams, searchParams]);

  const options : FilterOptions = useMemo(() => {
    const direction : any = searchParams.get('direction') || 'asc';
    const sort = searchParams.get('sort') || 'name';
    const name = searchParams.get('name');
    const categories = (searchParams.get('categories') || '')
      .split('-')
      .map(toNumber)
      .filter((id) => id > 0);

    return {
      categories,
      direction,
      sort,
      name,
    }
  }, [searchParams]);

  return { options, onSortChange };
}
