import React, { useEffect } from 'react';
import get from 'lodash/get';
import LoadingCell from './LoadingCell';
import { useLazyGetCategoryQuery } from '../../store/api';

export default function CategoryCell({ rowData, dataKey, ...props } : any) {
  const categoryId = get(rowData, dataKey, null);
  const [fetch, { data: category, isLoading }] = useLazyGetCategoryQuery();

  useEffect(() => {
    if (categoryId) {
      fetch(categoryId, true)
    }
  }, [categoryId])

  return (
    <LoadingCell loading={isLoading} {...props}>
      {categoryId ? category?.name : '-'}
    </LoadingCell>
  )
}
