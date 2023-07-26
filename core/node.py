from core.leafnode import LeafNode
from core.utils import midpoint
import math
import uuid

from settings import settings
MAX_LEVEL = settings['LoD']['max_level']
PROCESSES_PER_FRAME = settings['LoD']['processes_per_frame']
MIN_DISTANCE_MULTIPLIER = settings['LoD']['min_distance_multiplier']
MAX_DISTANCE_MULTIPLIER = settings['LoD']['max_distance_multiplier']
MIN_LEVEL_MLT = MAX_LEVEL - 3

class Node:
    def __init__(self, rect=[(0,0,0), (100,0,0), (100,0,100), (0,0,100)], level=1, parent=None, planet=None, tokill=None, toassign=None):
        self.rect = rect
        self.level = level
        self.parent = parent
        self.planet = planet
        self.tokill = tokill
        self.toassign = toassign
        self.children = []
        self._children = []
        self.position = [
            (self.rect[0][0]+self.rect[1][0]+self.rect[2][0]+self.rect[3][0])/4,
            (self.rect[0][1]+self.rect[1][1]+self.rect[2][1]+self.rect[3][1])/4,
            (self.rect[0][2]+self.rect[1][2]+self.rect[2][2]+self.rect[3][2])/4
        ]
        self.size = (
            math.dist(rect[0], self.position)+\
            math.dist(rect[1], self.position)+\
            math.dist(rect[2], self.position)+\
            math.dist(rect[3], self.position)
        )/4
        self.generated = False
        
        self.split_queue = []
        self.unify_queue = []
        self.id = uuid.uuid4()
        self.planet.children[self.id] = self
        self.type = None
        
    def swap_children(self):
        for child in self.children:
            self._children.append(child)
        self.children = []
        
    def remove_old_children(self):
        for tokill in self._children:
            tokill.dispose()
            del tokill
        self._children = []
        
    def generate_unified(self):
        lnode = LeafNode(self.rect, int(4+self.level/MAX_LEVEL), self, self.planet)
        self.children.append(lnode)
        self.kill_peers()
        self.type = "leaf"
        self.generated = True
        
    def generate_split(self):
        if self.level >= MAX_LEVEL:
            return # Don't split if we're at the max level
        # Split the quad into 4 quads
        corner1 = self.rect[0]
        corner2 = self.rect[1]
        corner3 = self.rect[2]
        corner4 = self.rect[3]
        
        # Make a midpoint between each corner
        midpoint1 = midpoint(corner1, corner2)
        midpoint2 = midpoint(corner2, corner3)
        midpoint3 = midpoint(corner3, corner4)
        midpoint4 = midpoint(corner4, corner1)
        midpoint5 = midpoint(corner1, corner3)
        
        # Now, make a simple quad
        rect1 = [corner1, midpoint1, midpoint5, midpoint4]
        rect2 = [midpoint1, corner2, midpoint2, midpoint5]
        rect3 = [midpoint5, midpoint2, corner3, midpoint3]
        rect4 = [midpoint4, midpoint5, midpoint3, corner4]
        
        # Create a node for each quad
        node1 = Node(rect=rect1, level=self.level + 1, parent=self, planet=self.planet)
        node2 = Node(rect=rect2, level=self.level + 1, parent=self, planet=self.planet)
        node3 = Node(rect=rect3, level=self.level + 1, parent=self, planet=self.planet)
        node4 = Node(rect=rect4, level=self.level + 1, parent=self, planet=self.planet)
        
        # Add the nodes to the children list
        self.children.append(node1)
        self.children.append(node2)
        self.children.append(node3)
        self.children.append(node4)
        
        # Add the nodes to the generation queue
        self.planet.generation_queue.append(node1)
        self.planet.generation_queue.append(node2)
        self.planet.generation_queue.append(node3)
        self.planet.generation_queue.append(node4)
        
        # Set some other properties
        self.type = "node"
        self.kill_peers()
        
    def kill_peers(self):
        try:
            if self.tokill:
                self.tokill.dispose()
                del self.tokill
                del self.parent.children[self.toassign]
                self.parent.children.insert(self.toassign, self)
            self.generated = True
        except:
            pass
        
    def draw(self):
        if len(self._children) > 0:
            for child in self._children:
                child.draw()
        else:
            for child in self.children:
                child.draw()
            
    def all_children_generated(self):
        for child in self.children:
            if not child.generated:
                return False
        return True
        
    def update(self):
        if len(self.position) == 0:
            return
        if self.type == "node":
            self.generated = self.all_children_generated()
            if self.generated:
                self.remove_old_children()
        # Calculate the distance between the player and the center of the quad
        distance = math.dist(self.planet.campos, self.position)
        
        # If the player is close enough to the quad, split it
        if self.level > MIN_LEVEL_MLT:
            mlt = abs(MIN_DISTANCE_MULTIPLIER + self.level / MIN_DISTANCE_MULTIPLIER * 2)
        else:
            mlt = MIN_DISTANCE_MULTIPLIER        
        if distance < self.size * mlt:
            if len(self.children) == 1 and self.level < MAX_LEVEL:
                self.parent.split_queue.append(self)
        elif distance > self.size * mlt:
            if not len(self.children) == 1:
                self.parent.unify_queue.append(self)
        
        for i in range(PROCESSES_PER_FRAME):
            # Process the split queue
            tosplit = self.split_queue.pop(0) if len(self.split_queue) > 0 else None
            if tosplit:
                tosplit.swap_children()
                tosplit.generate_split()
                    
            # Process the unify queue
            tounify = self.unify_queue.pop(0) if len(self.unify_queue) > 0 else None
            if tounify:
                rect = tounify.rect.copy()
                parent = tounify.parent
                planet = tounify.planet
                level = tounify.level
                try:
                    tounify_index = self.children.index(tounify)
                    tree = Node(rect, level, parent, planet=planet, tokill=tounify, toassign = tounify_index)
                    tree.generate_unified()
                except ValueError:
                    pass
        
    def distance_to(self, position):
        return math.dist(self.position, position)
                
    def dispose(self):
        try:
            del self.planet.children[self.id]
        except:
            pass
        for child in self.children:
            child.dispose()
        del self.children[:]
        self.children = []