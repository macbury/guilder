import { GuilderEndpointBuilder } from ".";

export type Category = {
  name: string,
  id: number,
}

export const buildCategoriesEndpoint = (builder : GuilderEndpointBuilder) => ({
  getCategories: builder.query<Category[], void>({
    query: () => `categories`,
    providesTags: ['Category'],
    transformResponse: (data : any) => data.categories
  }),

  getCategory: builder.query<Category, number | string>({
    query: (id) => `categories/${id}`,
    providesTags: ['Category'],
    transformResponse: (data : any) => data.category
  }),
})
