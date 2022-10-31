import React from 'react';
import { useGetCategoriesQuery } from '../../store/api';
import { useCategoriesState } from '../../store/hooks/categories';
import ObjectSelect, { IGenericObjectSelectProps } from './ObjectSelect';

export default function CategorySelect(props : IGenericObjectSelectProps) {
  const { data: categories } = useGetCategoriesQuery();
  const {
    actions: {
      newCategory
    }
  } = useCategoriesState();

  return (
    <ObjectSelect
      data={categories}
      newObject={newCategory}
      {...props} />
  )
}
