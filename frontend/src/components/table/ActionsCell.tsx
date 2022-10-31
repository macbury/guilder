import React, { useState, useCallback } from 'react';

import Table from 'rsuite/Table';
import IconButton from 'rsuite/IconButton';
import Divider from 'rsuite/Divider';
import EditIcon from '@rsuite/icons/Edit';
import TrashIcon from '@rsuite/icons/Trash';
import CloudReflashIcon from '@rsuite/icons/CloudReflash';

export interface IActionsCellProps {
  rowData?: any
  onSync?(id : number) : Promise<void>,
  onEdit?(id : number) : Promise<void>,
  onDestroy?(id : number) : Promise<void>,
  dataKey: string
}

export default function ActionsCell({ rowData, dataKey, onEdit, onDestroy, onSync, ...props } : IActionsCellProps) {
  const [loading, setLoading] = useState(false);
  const id = rowData[dataKey]

  const handleEdit = useCallback(async () => {
    setLoading(true);
    await onEdit(id);
    setLoading(false);
  }, [setLoading, id, onEdit]);

  const handleDestroy = useCallback(async () => {
    setLoading(true);
    if (confirm('Are you sure?')) {
      await onDestroy(id);
    }
    setLoading(false);
  }, [setLoading, id, onDestroy]);

  const handleSync = useCallback(async () => {
    setLoading(true);
    await onSync(id);
    setLoading(false);
  }, [setLoading, id, onSync]);

  return (
    <Table.Cell {...props} style={{ padding: 5 }} className="link-group">
      {onSync && <IconButton appearance="subtle" onClick={handleSync} icon={<CloudReflashIcon />} loading={loading} /> }
      {onSync && <Divider vertical />}
      {onEdit && <IconButton appearance="subtle" onClick={handleEdit} icon={<EditIcon />} loading={loading} /> }
      {onEdit && <Divider vertical />}
      {handleDestroy && <IconButton appearance="subtle" color="red" onClick={handleDestroy} icon={<TrashIcon />} loading={loading} /> }
    </Table.Cell>
  )
}
