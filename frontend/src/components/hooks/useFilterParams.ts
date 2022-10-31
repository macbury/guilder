import { useCallback } from 'react';
import toNumber from 'lodash/toNumber';
import { useSearchParams } from 'react-router-dom';

function useFilterParams(key : string) {
  const [searchParams, setSearchParams] = useSearchParams();
  const selectedIds = (searchParams.get(key) || '')
    .split('-')
    .map(toNumber)
    .filter((id) => id > 0);

  const setFilterParams = useCallback((categoryIds : number[]) => {
    searchParams.set(key, categoryIds.join('-'));
    setSearchParams(searchParams)
  }, [searchParams, key])

  return { selectedIds, setFilterParams };
}

export function useCategoriesParams() {
  return useFilterParams('categories')
}

export function useAccountsParams() {
  return useFilterParams('accounts')
}

export function useWalletsParams() {
  return useFilterParams('wallets')
}
