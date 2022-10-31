import React from 'react'
import Drawer from 'rsuite/Drawer';
import Button from 'rsuite/Button';
import Form from 'rsuite/Form';
import CurrencySelect from '../../components/inputs/CurrencySelect';
import { useAccountsState } from '../../store/hooks/accounts';

export default function AccountForm() {
  const {
    state: {
      loading,
      form: {
        accountId,
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
  } = useAccountsState();

  const existingRecord = !!accountId;
  const submit = existingRecord ? update : save;

  return (
    <Drawer open={visible} onClose={closeForm} backdrop="static">
      <Drawer.Header>
        <Drawer.Title>{existingRecord ? 'Edit Account' : 'New Account'}</Drawer.Title>
        <Drawer.Actions>
          <Button onClick={closeForm}>Cancel</Button>
          <Button onClick={submit} appearance="primary" loading={loading}>
            {existingRecord ? 'Update' : 'Save'}
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
          <Form.Group controlId="name">
            <Form.ControlLabel>Name:</Form.ControlLabel>
            <Form.Control
              errorMessage={errors['name']?.join(', ')}
              name="name" />
          </Form.Group>

          <Form.Group controlId="description">
            <Form.ControlLabel>Description:</Form.ControlLabel>
            <Form.Control
              errorMessage={errors['description']?.join(', ')}
              name="description" />
          </Form.Group>

          <Form.Group controlId="currency">
            <Form.ControlLabel>Currency:</Form.ControlLabel>
            <Form.Control
              size="lg"
              block
              disabled={existingRecord}
              accepter={CurrencySelect}
              errorMessage={errors['currency']?.join(', ')}
              name="currency" />
          </Form.Group>
        </Form>
      </Drawer.Body>
    </Drawer>
  )
}
