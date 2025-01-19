import pytest

from linked_list.linked_list import LinkedList, Node, EmptyListException

@pytest.fixture(scope="function")
def llist():
    yield LinkedList()

def test_linked_list(llist: LinkedList):
    assert llist.empty


def test_linked_list_append(llist: LinkedList):
    assert llist.empty

    llist.append(2)
    assert not llist.empty
    assert llist.size == 1
    assert llist.head.next.value == 2


def test_linked_list_pop(llist: LinkedList):
    assert llist.empty

    llist.append(2)
    assert not llist.empty
    popped_node: Node = llist.pop()
    assert popped_node.value == 2


def test_linked_list_pop_empty(llist: LinkedList):
    assert llist.empty

    with pytest.raises(EmptyListException):
        llist.pop()


def test_multiple_appends_pops(llist: LinkedList):
    array = [1,2,3,4,5]
    for a in array:
        llist.append(a)

    popped = []
    for _ in range(len(array)):
        popped.append(llist.pop().value)

    assert array ==  popped

