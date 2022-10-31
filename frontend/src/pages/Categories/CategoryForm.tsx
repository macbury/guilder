import React from 'react'
import Drawer from 'rsuite/Drawer';
import Button from 'rsuite/Button';
import Form from 'rsuite/Form';
import { useCategoriesState } from '../../store/hooks/categories';

export default function CategoryForm() {
  const {
    state: {
      loading,
      form: {
        categoryId,
        visible,
        value,
        errors
      }
    },

    actions: {
      closeForm,
      updateForm,
      save,
      update
    }
  } = useCategoriesState();

  const submit = categoryId ? update : save;

  return (
    <Drawer open={visible} onClose={() => closeForm()} backdrop="static">
      <Drawer.Header>
        <Drawer.Title>{categoryId ? 'Edit Category' : 'New Category'}</Drawer.Title>
        <Drawer.Actions>
          <Button onClick={() => closeForm()}>Cancel</Button>
          <Button onClick={submit} appearance="primary" loading={loading}>
            {categoryId ? 'Update' : 'Save'}
          </Button>
        </Drawer.Actions>
      </Drawer.Header>
      <Drawer.Body>
        <Form
          onSubmit={submit}
          disabled={loading}
          fluid
          formValue={value}
          onChange={updateForm}
        >
          <Form.Group controlId="input">
            <Form.ControlLabel>Name:</Form.ControlLabel>
            <Form.Control
              errorMessage={errors['name']?.join(', ')}
              name="name" />
          </Form.Group>
        </Form>
      </Drawer.Body>
    </Drawer>
  )
}
