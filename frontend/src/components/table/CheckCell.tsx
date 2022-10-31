import React, { useCallback } from 'react';
import Checkbox from 'rsuite/Checkbox';
import Table from 'rsuite/Table';

export interface ICheckCellProps {
  rowData?: any
  onChange?(checked: boolean, value: any)
  checkedKeys?: {
    [id: string | number]: boolean
  },
  dataKey: string
}

export default function CheckCell({ rowData, onChange, checkedKeys = {}, dataKey, ...props } : ICheckCellProps) {
  const id = rowData[dataKey];
  const checked = checkedKeys[id] || false;

  return (
    <Table.Cell {...props} style={{ padding: 0 }}>
      <div style={{ lineHeight: '46px' }}>
        <Checkbox
          style={{ margin: 0 }}
          value={rowData[dataKey]}
          inline
          onChange={(_, checked) => onChange(checked, id)}
          checked={checked}
        />
      </div>
    </Table.Cell>
  )
}

export interface ICheckCellHeaderProps {
  selectedCount: number
  totalCount: number
  selectAll()
  deselectAll()
}

export function CheckCellHeader({ selectedCount, totalCount, selectAll, deselectAll, ...props} : ICheckCellHeaderProps) {
  const checked = selectedCount == totalCount && selectedCount > 0;
  const indeterminate = selectedCount < totalCount && selectedCount > 0;
  const handleCheckAll = useCallback(() => {
    if (checked) {
      deselectAll()
    } else {
      selectAll()
    }
  }, [checked])

  return (
    <div style={{ lineHeight: '40px' }} {...props}>
      <Checkbox
        inline
        style={{ margin: 0 }}
        checked={checked}
        indeterminate={indeterminate}
        onChange={handleCheckAll}
      />
    </div>
  )
}
