class MyTree:

    class TreeNode:
        def __init__(self, data = None):
            self.data = data
            self.left = None
            self.right = None

    def __init__(self, size = 1):
        self.root = MyTree.TreeNode()
        self.size = size

class MyGraph:

    class Edge:
        def __init__(self):
            self.to = None
            self.weight = None

    def __init__(self):
        self.adjacency_table = {}

class MyBST:
    class BSTreeNode:

        def __init__(self, data = None):
            self.data = data
            self.left = None
            self.right = None
            self.size = 0

    def __init__(self):
        self.root = self.BSTreeNode()
        
    def traverse(self, node):
        if node is None:
            return
        
        # pre-order
        print(node)
        
        self.traverse(node.left)
        # in-order
        print(node)
        
        self.traverse(node.right)
        # post-order
        print(node)

    def search(self, node, target):
        if node is None:
            return False

        if node.data == target:
            return True
        elif target < node.data:
            return self.search(node.left, target)
        else:
            return self.search(node.right, target)

    def search_iterative(self, target):

        node = self.root

        while node is not None:
            if node.data == target:
                return True
            elif target < node.data:
                node = node.left
            else:
                node = node.right

        return False


    def put(self, data):
        pass

    def remove(self):
        pass

    def max(self):
        pass

    def min(self):
        pass

    def sorted_list(self):
        pass

    def flooring(self, value):
        pass

    def ceiling(self, value):
        pass

    def rank(self, rank):
        pass

    def _right_rotate(self, node):
        pass

    def _left_rotate(self, node):
        pass

class MyHeap:
    def __init__(self):
        self.heap = []
