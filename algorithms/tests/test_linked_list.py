import pytest

from linked_list.linked_list import LinkedList, Node, EmptyListException

@pytest.fixture(scope="function")
def llist():
    yield LinkedList()

@pytest.fixture(scope="function")
def filled_llist():
    llist = LinkedList()
    array = [1,2,3,4,5]
    for a in array:
        llist.append(a)
    yield llist


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
    popped_node: Node = llist.pop_head()
    assert popped_node.value == 2


def test_linked_list_pop_empty(llist: LinkedList):
    assert llist.empty

    with pytest.raises(EmptyListException):
        llist.pop_head()


def test_multiple_appends_pops(llist: LinkedList):
    array = [1,2,3,4,5]
    for a in array:
        llist.append(a)

    popped = []
    for _ in range(len(array)):
        popped.append(llist.pop_head().value)

    assert array ==  popped


def test_get(filled_llist: LinkedList):    
    for i in range(len(filled_llist)):
        filled_llist.get(i) == i + 1


def test_get_negative_index(filled_llist: LinkedList):    
    with pytest.raises(IndexError):
        filled_llist.get(-1)

def test_get_out_of_bounds_index(filled_llist: LinkedList):    
    with pytest.raises(IndexError):
        filled_llist.get(5)


def test_delete(filled_llist: LinkedList):    
    node = filled_llist.delete(2)
    assert node.value == 3
    assert len(filled_llist) == 4

    collected = []
    for i in range(4):
        collected.append(filled_llist.get(i).value)

    assert collected == [1, 2, 4, 5]


def test_iter(filled_llist: LinkedList):
    array = [1,2,3,4,5]
    for idx, value in enumerate(filled_llist):
        assert value == array[idx]
