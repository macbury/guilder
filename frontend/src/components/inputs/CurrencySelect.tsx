import React, { useCallback, useMemo } from 'react'
import SelectPicker from 'rsuite/SelectPicker';
import useAxios from 'axios-hooks';
import SpinnerIcon from '@rsuite/icons/legacy/Spinner';

export interface ICurrencySelectProps {

}

export default function CurrencySelect(props : ICurrencySelectProps) {
  const [{ data, loading, error }, refetch] = useAxios('/api/currencies')

  const renderMenu = useCallback((menu) => {
    if (loading) {
      return (
        <p style={{ padding: 4, color: '#999', textAlign: 'center' }}>
          <SpinnerIcon spin /> Loading...
        </p>
      );
    }
    return menu;
  }, [loading]);

  const currencies = useMemo(() => {
    return (data || []).map(({ alpha3, name, symbol }) => {
      return {
        value: alpha3,
        label: `[${alpha3}] ${name} (${symbol})`
      }
    })
  }, [data])

  return (
    <SelectPicker
      data={currencies}
      renderMenu={renderMenu}
      {...props}
    />
  )
}
