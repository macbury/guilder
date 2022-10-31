import React from 'react';
import { useCategoriesState } from '../../store/hooks/categories';
import { useGetCategoriesQuery } from '../../store/api';
import GenericCheckboxFilter from './GenericCheckboxFilter';

export interface ICategoriesFilterProps {
  selectedCategoryIds: number[],
  onTickCategory(selectedCategoryIds: number[]) : void
}

export default function CategoriesFilter({ selectedCategoryIds, onTickCategory  } : ICategoriesFilterProps) {
  const { data: categories, isLoading } = useGetCategoriesQuery();

  const {
    actions: { newCategory }
  } = useCategoriesState();

  return (
    <GenericCheckboxFilter
      header="Category"
      items={categories}
      loading={isLoading}
      newItem={newCategory}
      onTickItem={onTickCategory}
      selectedItemsIds={selectedCategoryIds}
    />
  )
}
