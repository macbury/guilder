import React from 'react'
import { useGetCategoryQuery } from '../../store/api';
import { Row } from './Row';

export interface ICategoryRowProps {
  name: string,
  categoryId: number
}

export function CategoryRow({ name, categoryId, ...props } : ICategoryRowProps) {
  const { data: category } = useGetCategoryQuery(categoryId);

  return (
    <Row name={name} {...props}>
      {category?.name || '-'}
    </Row>
  )
}
