import React, { useMemo } from 'react'
import Drawer from 'rsuite/Drawer';
import Button from 'rsuite/Button';
import Form from 'rsuite/Form';
import { useAssetsState } from '../../store/hooks/assets';
import CategorySelect from '../../components/inputs/CategorySelect';

export default function AssetForm() {

  const {
    state: {
      form: {
        visible,
        errors,
        value
      },
      loading,
      selectedIds
    },
    actions: {
      hideForm,
      updateForm,
      massUpdate
    }
  } = useAssetsState();

  return (
    <Drawer open={visible} onClose={hideForm} backdrop="static" enforceFocus placement="left">
      <Drawer.Header>
        <Drawer.Title>Batch action</Drawer.Title>
        <Drawer.Actions>
          <Button onClick={hideForm}>Cancel</Button>
          <Button onClick={massUpdate} appearance="primary" loading={loading}>
            Update {selectedIds.length} assets
          </Button>
        </Drawer.Actions>
      </Drawer.Header>
      <Drawer.Body>
        <Form
          onSubmit={massUpdate}
          disabled={loading}
          fluid
          formValue={value}
          onChange={updateForm}
        >
          <Form.Group controlId="input">
            <Form.ControlLabel>Category:</Form.ControlLabel>
            <CategorySelect
              errors={errors}
              name="categoryId"
            />
          </Form.Group>
        </Form>
      </Drawer.Body>
    </Drawer>
  )
}
