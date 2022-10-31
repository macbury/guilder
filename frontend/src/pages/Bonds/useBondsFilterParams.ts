import { useMemo } from "react";
import toNumber from 'lodash/toNumber'
import { useSearchParams } from "react-router-dom";
import { useFilterParams } from "../../hooks/useFilterParams";
import { BondFilterOptions } from "../../store/api/bonds";

function getParamIds(searchParams, key) {
  return (searchParams.get(key) || '')
    .split('-')
    .map(toNumber)
    .filter((id) => id > 0)
}

export default function useBondsFilterParams() {
  const [searchParams] = useSearchParams();
  const { options, onSortChange } = useFilterParams();
  const scope = searchParams.get('scope') as any || 'active';
  const accounts = getParamIds(searchParams, 'accounts');
  const wallets = getParamIds(searchParams, 'wallets');

  const bondOptions : BondFilterOptions = useMemo(() => ({
    ...options,
    scope,
    accounts,
    wallets
  }), [options, scope]);

  return { options: bondOptions, onSortChange }
}
